import {Model} from "./model.js";

const vec2 = glMatrix.vec2;
const mat2d = glMatrix.mat2d;
const mat4 = glMatrix.mat4;

function loadSource(url) {
    return new Promise((resolve, reject) => {
        let request = new XMLHttpRequest;
        request.open("get", url);
        request.onload = () => {
            if (200 <= request.status && request.status < 300) {
                resolve(request.response);
            } else {
                reject({
                    status: request.status,
                    text: request.statusText,
                });
            }
        };
        request.onerror = () => {
            reject({
                status: request.status,
                text: request.statusText,
            });
        };
        request.send();
    });
}

class ProgramInfo {
    async setUp(gl, sources) {
        this.gl = gl;

        await this.loadProgram(sources);
        this.fetchLocations();
    }

    async loadProgram(sources) {
        const shaders = [
            this.loadShader(this.gl.VERTEX_SHADER, await sources.vertex),
            this.loadShader(this.gl.FRAGMENT_SHADER, await sources.fragment),
        ];

        this.program = this.gl.createProgram();
        for (const shader of shaders) {
            this.gl.attachShader(this.program, await shader);
        }
        this.gl.linkProgram(this.program);

        if (!this.gl.getProgramParameter(this.program, this.gl.LINK_STATUS)) {
            const log = this.gl.getProgramInfoLog(this.program);
            this.gl.deleteProgram(this.program);
            throw {
                shaders: shaders,
                log: log,
            };
        }
    }

    async loadShader(type, source) {
        const shader = this.gl.createShader(type);
        this.gl.shaderSource(shader, source);
        this.gl.compileShader(shader);

        if (!this.gl.getShaderParameter(shader, this.gl.COMPILE_STATUS)) {
            const log = this.gl.getShaderInfoLog(shader);
            this.gl.deleteShader(shader);
            throw {
                type: type,
                source: source,
                log: log,
            };
        }

        return shader;
    }

    fetchLocations() {
        this.uniformLocations = {};
        const uniformCount = this.gl.getProgramParameter(this.program, this.gl.ACTIVE_UNIFORMS);
        for (let uniformIndex = 0; uniformIndex < uniformCount; uniformIndex++) {
            const uniform = this.gl.getActiveUniform(this.program, uniformIndex);
            const name = uniform.name.substr(2);
            this.uniformLocations[name] = this.gl.getUniformLocation(this.program, uniform.name);
        }

        this.attributeLocations = {};
        const attributeCount = this.gl.getProgramParameter(this.program, this.gl.ACTIVE_ATTRIBUTES);
        for (let attributeIndex = 0; attributeIndex < attributeCount; attributeIndex++) {
            const attribute = this.gl.getActiveAttrib(this.program, attributeIndex);
            const name = attribute.name.substr(2);
            this.attributeLocations[name] = this.gl.getAttribLocation(this.program, attribute.name);
        }
    }
}

class Renderer {
    constructor(shaderData) {
        this.shaderData = shaderData;
        this.programInfo = new ProgramInfo();
    }

    async setUp(gl, sources) {
        this.gl = gl;
        await Promise.all([
            this.programInfo.setUp(gl, sources),
            this.createBuffers(),
        ]);
    }

    get uniformLocations() {
        return this.programInfo.uniformLocations;
    }

    get attributeLocations() {
        return this.programInfo.attributeLocations;
    }
}

class StationRenderer extends Renderer {
    createBuffers() {
        this.buffers = {
            position: this.gl.createBuffer(),
        }
    }

    fillBuffers(model) {
        const positions = model.stations.reduce((positions, station) => {
            return positions.concat(station.vertices);
        }, []);
        this.gl.bindBuffer(this.gl.ARRAY_BUFFER, this.buffers.position);
        this.gl.bufferData(this.gl.ARRAY_BUFFER, new Float32Array(positions), this.gl.STATIC_DRAW);

        this.size = model.stations.length;
    }

    run() {
        this.gl.useProgram(this.programInfo.program);

        this.gl.uniform1f(this.uniformLocations.size, 5.0);
        this.gl.uniformMatrix4fv(this.uniformLocations.modelView, false, this.shaderData.uniforms.modelView);

        this.gl.bindBuffer(this.gl.ARRAY_BUFFER, this.buffers.position);
        this.gl.vertexAttribPointer(
            this.attributeLocations.position,
            2, this.gl.FLOAT,
            false, 0, 0);
        this.gl.enableVertexAttribArray(this.attributeLocations.position);

        this.gl.drawArrays(this.gl.POINTS, 0, this.size);
    }
}

class LineRenderer extends Renderer {
    createBuffers() {
        this.buffers = {
            position: this.gl.createBuffer(),
        }
    }

    fillBuffers(model) {
        const positions = model.lines.reduce((positions, line) => {
            return positions.concat(line.vertices);
        }, []);
        this.gl.bindBuffer(this.gl.ARRAY_BUFFER, this.buffers.position);
        this.gl.bufferData(this.gl.ARRAY_BUFFER, new Float32Array(positions), this.gl.STATIC_DRAW);

        this.sizes = model.lines.map(line => 2 * line.stops.length);
        this.colors = model.lines.map(line => line.color.map(component => component / 255));
    }

    run() {
        this.gl.useProgram(this.programInfo.program);

        this.gl.uniformMatrix4fv(this.uniformLocations.modelView, false, this.shaderData.uniforms.modelView);

        this.gl.bindBuffer(this.gl.ARRAY_BUFFER, this.buffers.position);
        this.gl.vertexAttribPointer(
            this.attributeLocations.position,
            2, this.gl.FLOAT,
            false, 0, 0);
        this.gl.enableVertexAttribArray(this.attributeLocations.position);

        this.sizes.reduce((first, count, index) => {
            this.gl.uniform3fv(this.uniformLocations.color, this.colors[index]);
            this.gl.drawArrays(this.gl.TRIANGLE_STRIP, first, count);
            return first + count;
        }, 0);
    }
}

class ShaderData {
    async setUp(gl) {
        this.canvas = gl.canvas;
        this.initMatrices();
    }

    initMatrices() {
        this.uniforms = {
            model: glMatrix.mat2d.create(),
            view: glMatrix.mat2d.create(),
            modelView: glMatrix.mat4.create(),
        };

        mat2d.scale(this.uniforms.model, this.uniforms.model, vec2.fromValues(2000.0, 4000.0));
        mat2d.translate(this.uniforms.model, this.uniforms.model, vec2.fromValues(-13.5, -52.53));

        this.calculateModelView();
    }

    translateView(x, y) {
        const view = mat2d.create();
        mat2d.fromTranslation(view, vec2.fromValues(2 * x, -2 * y));
        mat2d.multiply(this.uniforms.view, view, this.uniforms.view);
        this.calculateModelView();
    }

    scaleView(scale, x, y) {
        const translation = vec2.fromValues(2 * (x - this.canvas.width / 2), -2 * (y - this.canvas.height / 2));
        const view = mat2d.create();
        mat2d.translate(view, view, translation);
        mat2d.scale(view, view, vec2.fromValues(scale, scale));
        mat2d.translate(view, view, vec2.negate(translation, translation));
        mat2d.multiply(this.uniforms.view, view, this.uniforms.view);
        this.calculateModelView();
    }

    calculateModelView() {
        const modelView2d = mat2d.create();
        mat2d.scale(modelView2d, modelView2d, vec2.fromValues(1.0 / this.canvas.width, 1.0 / this.canvas.height));
        mat2d.multiply(modelView2d, modelView2d, this.uniforms.view);
        mat2d.multiply(modelView2d, modelView2d, this.uniforms.model);

        mat4.identity(this.uniforms.modelView);
        this.uniforms.modelView[0] = modelView2d[0];
        this.uniforms.modelView[1] = modelView2d[1];
        this.uniforms.modelView[4] = modelView2d[2];
        this.uniforms.modelView[5] = modelView2d[3];
        this.uniforms.modelView[12] = modelView2d[4];
        this.uniforms.modelView[13] = modelView2d[5];
    }
}

class Controller {
    async setUp(gl) {
        this.gl = gl;
        this.initializeCanvas();

        this.model = new Model();
        this.shaderData = new ShaderData();
        this.renderer = {
            line: new LineRenderer(this.shaderData),
            station: new StationRenderer(this.shaderData),
        };

        await Promise.all([
            this.model.setUp(sources.data),
            this.shaderData.setUp(this.gl),
            this.renderer.line.setUp(this.gl, sources.line),
            this.renderer.station.setUp(this.gl, sources.station),
        ]);
        await Promise.all([
            this.renderer.line.fillBuffers(this.model),
            this.renderer.station.fillBuffers(this.model),
        ]);
        this.drawLoop();
        this.addControlListeners();
    }

    initializeCanvas() {
        this.resizeCanvas();
        addEventListener("resize", () => this.resizeCanvas());
    }

    addControlListeners() {
        addEventListener("resize", () => this.shaderData.calculateModelView());
        addEventListener("mousemove", event => {
            if (event.buttons) {
                this.shaderData.translateView(event.movementX, event.movementY);
            }
        });
        addEventListener("wheel", event => {
            if (event.deltaY < 0) {
                this.shaderData.scaleView(11 / 10, event.clientX, event.clientY);
            } else {
                this.shaderData.scaleView(10 / 11, event.clientX, event.clientY);
            }
        });
    }

    resizeCanvas() {
        const canvas = this.gl.canvas;
        canvas.width = document.body.clientWidth;
        canvas.height = document.body.clientHeight;
        this.gl.viewport(0, 0, canvas.width, canvas.height);
        this.clear();
    }

    clear() {
        this.gl.clearColor(0.85, 0.9, 0.9, 1.0);
        this.gl.clear(this.gl.COLOR_BUFFER_BIT);
    }

    drawLoop() {
        this.draw();
        requestAnimationFrame(() => this.drawLoop());
    }

    draw() {
        this.gl.disable(this.gl.DEPTH_TEST);
        this.gl.enable(this.gl.BLEND);
        this.gl.blendFuncSeparate(this.gl.SRC_ALPHA, this.gl.ONE_MINUS_SRC_ALPHA, this.gl.ZERO, this.gl.ONE);

        this.clear();
        this.renderer.line.run();
        this.renderer.station.run();
    }
}

const sources = {
    line: {
        vertex: loadSource("shader/line.vert.glsl"),
        fragment: loadSource("shader/line.frag.glsl"),
    },
    station: {
        vertex: loadSource("shader/station.vert.glsl"),
        fragment: loadSource("shader/station.frag.glsl"),
    },
    data: loadSource("../data/vbb.json"),
};

const controller = new Controller();

addEventListener("load", () => {
    const canvas = document.querySelector("canvas");
    const gl = canvas.getContext("webgl", {alpha: false});
    controller.setUp(gl).catch(error => console.error(error));
});
