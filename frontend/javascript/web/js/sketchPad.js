class SketchPad{
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
    this.paths = [];
    this.isDrawing = false;
    this.historyStorage = [];

    this.#addEventListeners();
  }

  #addEventListeners() {
    this.canvas.onmousedown = (evt) => {
        this.historyStorage.length = 0;
        const mouse = this.#getmouse(evt);
        this.paths.push([mouse]);
        this.isDrawing = true;
    }
    this.canvas.onmousemove = (evt) => {
      if (this.isDrawing) {
        const mouse = this.#getmouse(evt);
        const currentPath = this.paths[this.paths.length-1];
        currentPath.push(mouse);
        this.#redraw();
      }
    }
    this.canvas.onmouseup = () => {
      this.#redraw();
      this.isDrawing = false;
    };
    this.canvas.ontouchstart=(evt)=>{
      const loc=evt.touches[0];
      this.canvas.onmousedown(loc);
    };
    this.canvas.ontoucnemove=(evt)=>{
      const loc=evt.touches[0];
      this.canvas.onmousemove(loc);
    };
    this.canvas.ontouchend =()=>{
      this.canvas.onmouseup();
    }
    this.canvas.onmouseout =()=>{
      this.canvas.onmouseup();
    }
  }
  #redraw() {
    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
    draw.paths(this.ctx, this.paths);
  }
  redo(){
    if (this.historyStorage.length > 0) {
    this.paths.push(this.historyStorage.pop());
    this.#redraw();
  }
  }
  undo(){
    if (this.paths.length >= 1) {
    this.historyStorage.push(this.paths.pop());
    this.#redraw();
  }}
  haveReset() {
    this.paths.length = 0;
    this.ctx.clearRect(0,0,this.canvas.width, this.canvas.height);
  };

  #getmouse = (evt) => {
    const rect = this.canvas.getBoundingClientRect();
    return [
      Math.round(evt.clientX - rect.left),
      Math.round(evt.clientY - rect.top)
    ];
  }
}

// const sketchPad = new SketchPad(document.getElementById("sketchPadContainer"));

