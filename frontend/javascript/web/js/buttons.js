const rButton = document.getElementById("resetButton");
rButton.addEventListener("click", function(){
  sketchPad.haveReset();
});

const uButton =document.getElementById("undoButton");
uButton.addEventListener("click", function(){
  sketchPad.undo();
})

const reButton =document.getElementById("redoButton");
reButton.addEventListener("click", function(){
  console.log("redo click");
  sketchPad.redo();
})

document.getElementById('userName').onkeydown = function(e){
  if(e.keyCode ==13){
    document.getElementById('advanceBtn').click();
  };
}
