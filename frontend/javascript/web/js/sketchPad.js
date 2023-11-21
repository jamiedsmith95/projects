class SketchPad {
  constructor(container, size = 400) {
    this.canvas = document.createElement("canvas");
    this.canvas.width = size;
    this.canvas.height = size;
    this.canvas.style = `
background-color:white;
box-shadow: 0px 0px 10px 2px black;
`;
    container.appendChild(this.canvas);

    this.ctx = this.canvas.getContext("2d");
    this.path = [];
    this.isDrawing = false;

    this.#addEventListeners();
  }
  haveReset() {
    this.path = [];
  }

  #addEventListeners() {
    this.canvas.onmousedown = (evt) => {
      this.isDrawing = true;
      if (this.isDrawing) {
        const mouse = this.#getmouse(evt);
        this.path.push(mouse);
      }
    }
    this.canvas.onmousemove = (evt) => {
      if (this.isDrawing) {
        this.isDrawing = true;
        const mouse = this.#getmouse(evt);
        this.path.push(mouse);
        this.#redraw();
      }
    }
    this.canvas.onmouseup = () => {
      this.isDrawing = false;
    }
  }
  #redraw() {
    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
    draw.path(this.ctx, this.path);
  }
  #getmouse = (evt) => {
    const rect = this.canvas.getBoundingClientRect();
    return [
      Math.round(evt.clientX - rect.left),
      Math.round(evt.clientY - rect.top)
    ];
  }
}
