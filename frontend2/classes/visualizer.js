// sorry guys, but sciter does not support fill in #FFFFFFFF format
function hex2rgba(hex) {
    var r = parseInt(hex.substring(1, 3), 16) / 255;
    var g = parseInt(hex.substring(3, 5), 16) / 255;
    var b = parseInt(hex.substring(5, 7), 16) / 255;
    var alpha = 1.0;
    return "rgba(" + r + ", " + g + ", " + b + ", " + alpha + ")";
}


class Visualizer {
    constructor(color) {
        this.canvas = $("canvas")[0];
        this.bufferLength = 128;
        this.ctx = this.canvas.getContext("2d");
        this.sampleData = [];
        this.color = hex2rgba(color);
        this.enabled = true;
    }

    update() {
        if (this.enabled) {
            this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
            this.sampleData = Window.this.xcall("get_samples");
            let x = 0;

            if (!this.sampleData || this.sampleData <= 0) {
                return;
            }

            const barWidth = Math.floor(this.canvas.width / this.bufferLength);

            for (let i = 0; i < this.bufferLength; i++) {
                let barHeight = this.sampleData[i] * this.canvas.width;
                this.ctx.fillStyle = this.color;
                this.ctx.fillRect(x, this.canvas.height - barHeight, barWidth, barHeight);
                x += barWidth;
            }

            // const scale = clamp(dataArray[0] * 5.0, 1.0, 1.2);
            // $("#now-playing-img")[0].style.transform = `scale(${scale}, ${scale})`;
        }
    }
}