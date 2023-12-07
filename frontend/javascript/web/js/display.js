function createRow(container,userName,samples){
  const row=document.createElement("div");
  row.classList.add("row");
  container.appendChild(row);

  const rowLabel=document.createElement("div");
  rowLabel.innerHTML=userName;
  rowLabel.classList.add("rowLabel");
  row.appendChild(rowLabel);

  for(let sample of samples){
    const {id,label}=sample;

    const sampleContainer=document.createElement("div");
    sampleContainer.id="sample_"+id;
    sampleContainer.classList.add("sampleContainer")

    const sampleLabel = document.createElement("div")
    sampleLabel.id="sampleLabel"
    sampleLabel.innerHTML=label;
    sampleContainer.appendChild(sampleLabel);
    const img=document.createElement('img');
    img.src=constants.IMG_DIR+'/'+id+'.png';
    img.classList.add("thumb");
    sampleContainer.appendChild(img);
    row.appendChild(sampleContainer);
  }
}
