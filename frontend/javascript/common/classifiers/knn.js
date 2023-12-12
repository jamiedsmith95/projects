if(typeof utils==='undefined'){
const utils = require('../utils');
}

class KNN {
  constructor(samples, k) {
    this.samples = samples;
    this.k = k;
  }
  predict(point) {
    const samplePoints = this.samples.map(s => s.point);
    const indexs = utils.getKNearest(point, samplePoints, this.k);
    let labels = []
    for (let i = 0; i < indexs.length; i++) {
      const idx = indexs[i];
      labels.push(this.samples[idx].label);
    }
    const label = utils.getMode(labels);
    return label;
  }

}

if(typeof module!=='undefined'){
  module.exports=KNN;
}
