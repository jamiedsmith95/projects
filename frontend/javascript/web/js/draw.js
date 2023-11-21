const draw = {};
draw.path = (ctx, path, color = "black") => {
  ctx.strokeStyle = color;
  ctx.linewidth = 3;
  ctx.moveTo(...path[0]);
  ctx.beginPath();
  for (let i = 1; i < path.length; i++) {
    ctx.lineTo(...path[i]);
  }
  ctx.stroke();
}
