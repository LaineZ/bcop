class Visualizer {
    constructor() {
        this.canvas = $("canvas")[0];
        this.bufferLength = 128;
        this.ctx = this.canvas.getContext("2d");
        this.sampleData = [];
    }

    update() {
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        this.sampleData = Window.this.xcall("get_samples");
        let x = 0;

        if (!this.sampleData || this.sampleData <= 0) {
            return;
        }

        const barWidth = Math.floor(this.canvas.width / this.bufferLength);    

        for (let i = 0; i < this.bufferLength; i++) {
            let barHeight = this.sampleData[i] * this.canvas.width;
            this.ctx.fillStyle = "rgb(1.0,0.8,0.7)";
            this.ctx.fillRect(x, this.canvas.height - barHeight, barWidth, barHeight);
            x += barWidth;
        }

        // const scale = clamp(dataArray[0] * 5.0, 1.0, 1.2);
        // $("#now-playing-img")[0].style.transform = `scale(${scale}, ${scale})`;
    }
}