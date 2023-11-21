class resetButton {
  constructor(container, size = 20) {
    this.button = document.createElement("button"); 
    this.button.width = size;
    this.button.height = size;
    this.button.style = `
background-color:black;
`
    this.button.pressed = false;
    container.appendChild(this.button);
    this.#addEventListeners();
    
  }
  #addEventListeners() {
    this.button.onclick = () => {
      sketchPad.haveReset();

    }
  }
}



