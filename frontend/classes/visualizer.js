class Visualizer {
    constructor(width, height) {
        this.canvas = $("canvas")[0];
        this.canvas.width = width;
        this.canvas.height = height;
        this.bufferLength = 256;
        this.ctx = this.canvas.getContext("2d");
    }

    update() {
        let dataArray = Window.this.xcall("get_samples");
        let x = 0;

        if (!dataArray || dataArray.length <= 0) {
            return;
        }
        
        const barWidth = this.canvas.width / this.bufferLength;    

        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        for (let i = 0; i < this.bufferLength; i++) {
            let barHeight = dataArray[i] * this.canvas.width;
            this.ctx.fillStyle = "rgb(1.0,0.8,0.7)";
            this.ctx.fillRect(x, this.canvas.height - barHeight, barWidth, barHeight);
            x += barWidth;
        }
    }
}