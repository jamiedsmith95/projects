const constants = require('../common/constants.js');
const utils = require('../common/utils.js');

const KNN = require('../common/classifiers/knn.js');

const fs = require('fs');

console.log("RUNNING CLASSIFICATION");

const { samples: trainingSamples } = JSON.parse(
  fs.readFileSync(constants.TRAINING)
);

const kNN = new KNN(trainingSamples, 50);

const { samples: testingSamples } = JSON.parse(
  fs.readFileSync(constants.TESTING)
);

let totalCount = 0;
let correctCount = 0;
for (const sample of testingSamples) {
  const predictedLabel = kNN.predict(sample.point);
  correctCount += (predictedLabel == sample.label);
  totalCount += 1;
}

console.log("ACCURACY: " + correctCount + "/" + totalCount + " (" + utils.formatPercent(correctCount / totalCount) + ")");


console.log("GENERATING DECISION BOUNDARY ...");

const {createCanvas}=require('canvas');
const canvas=createCanvas(100,100);
const ctx=canvas.getContext('2d');

//TODO colour each pixel according to its predictedLabel.
// for(let x=0;x<canvas.width;x++){
//   for(let y=0;y<canvas.height;y++){

//   }
// }
