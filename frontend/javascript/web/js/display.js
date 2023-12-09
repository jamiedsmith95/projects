function createRow(container, userName, samples) {
  const row = document.createElement("div");
  row.classList.add("row");
  container.appendChild(row);

  const rowLabel = document.createElement("div");
  rowLabel.innerHTML = userName;
  rowLabel.classList.add("rowLabel");
  row.appendChild(rowLabel);

  for (let sample of samples) {
    const { id, label } = sample;

    const sampleContainer = document.createElement("div");
    sampleContainer.id = "sample_" + id;
    sampleContainer.onclick = () => {
      handleClick(sample, false);
    }
    sampleContainer.onmouseover = () =>
      handleHover(sample)
    sampleContainer.classList.add("sampleContainer")

    const sampleLabel = document.createElement("div")
    sampleLabel.id = "sampleLabel"
    sampleLabel.innerHTML = label;
    sampleContainer.appendChild(sampleLabel);
    const img = document.createElement('img');
    img.src = constants.IMG_DIR + '/' + id + '.png';
    img.classList.add("thumb");
    sampleContainer.appendChild(img);
    row.appendChild(sampleContainer);
  }
}

function handleHover(sample) {
  if (sample == null) {
    [...document.querySelectorAll('.hovered')].forEach((e) => e.classList.remove('hovered'));

    return
  }
  [...document.querySelectorAll('.hovered')].forEach((e) => e.classList.remove('hovered'));
  const el = document.getElementById("sample_" + sample.id);
  if (el.classList.contains('emphasize')) {
    el.parentElement.classList.add('hovered');
    el.classList.add('hovered');
    el.scrollIntoView({
      behavior: 'smooth',
      block: 'center'
    });
  };
}

function handleClick(sample, doScroll = true) {
  if (!sample) {
    [...document.querySelectorAll('.emphasize')].forEach((e) => e.classList.remove('emphasize'));
  } else {
    [...document.querySelectorAll('.emphasize')].forEach((e) => e.classList.remove('emphasize'));

    const el = document.getElementById("sample_" + sample.id);
    el.classList.add('emphasize');
    if (doScroll) {
      el.scrollIntoView({
        behavior: 'auto',
        block: 'center'
      });
    }
    chart.selectSample(sample);
  }
}
