const featureFunctions={};

featureFunctions.getWidth=(paths)=>{
  const points=paths.flat();
  const x=points.map(p=>p[0]);
  const min=Math.min(...x);
  const max=Math.max(...x);
  if (max-min>400){
    return 400
  }
  return max-min;
}
featureFunctions.getHeight=(paths)=>{
  const points=paths.flat();
  const y=points.map(p=>p[1]);
  const min=Math.min(...y);
  const max=Math.max(...y);
  if (max-min>400){
    return 400
  }
  return max-min;
}
featureFunctions.getPathCount=(paths)=>{
  return paths.length;
}

featureFunctions.getPointCount=(paths)=>{
  const points = paths.flat();
  return points.length;
}

featureFunctions.inUse=[
  {name:"Width",function:featureFunctions.getWidth},
  {name:"Height",function:featureFunctions.getHeight}
];


if(typeof module!=='undefined'){
  module.exports=featureFunctions;
}