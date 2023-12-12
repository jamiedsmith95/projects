const utils = {};

utils.formatPercent = (n) => {
  return (n * 100).toFixed(2) + "%";
}

utils.distance = (p1, p2) => {
  let sum = 0;
  for (let i = 0; i < p1.length; i++) {
    sum += (p1[i] - p2[i]) ** 2
  }
  return Math.sqrt(sum);
}
// could also implement k nearest rather than radius selection. also need to generalise to n dimensions.
utils.getKNearest = (loc, points, k) => {
  let list = [];
  for (let i = 0; i < points.length; i++) {
    const dist = utils.distance(loc, points[i]);
    list.push({ 'idx': i, 'dist': dist })
  }
  list.sort(function(a, b) {
    return ((a.dist < b.dist) ? -1 : ((a.dist == b.dist) ? 0 : 1));
  })
  let idxs = [];
  for (let i = 0; i < k; i++) {
    idxs.push(list[i].idx);

  }

  return idxs;
}

utils.getMode = (labels) => {
  const counts = {};

  for (let i = 0; i < labels.length; i++) {
    if (counts[labels[i]]) {
      counts[labels[i]] += 1;
    } else {
      counts[labels[i]] = 1;
    }
  }
  let leaderVal = -1;
  let leaderKey = -1;

  Object.keys(counts).forEach(key => {
    let value = counts[key];
    if (value > leaderVal) {
      leaderVal = value;
      leaderKey = key;
    }
  });
  return leaderKey
}

utils.invLerp = (a, b, v) => {
  return (v - a) / (b - a);
}

utils.normalisePoint = (points, minMax) => {
  let min, max = [];
  if (minMax) {
    min = minMax.min;
    max  = minMax.max;
  } else {
    min = [...points[0]];
    max = [...points[0]];
    for (let i = 1; i < points.length; i++) {
      for (let j = 0; j < min.length; j++) {
        if (points[i][j] > max[j]) {
          max[j] = points[i][j];
        }
        if (points[i][j] < min[j]) {
          min[j] = points[i][j]
        }
      }

    }
  };
  for (let i = 0; i < points.length; i++) {
    for (let j = 0; j < min.length; j++) {
      points[i][j] = utils.invLerp(min[j], max[j], points[i][j]);
    }
  }

  return { min, max };
}

utils.getNearest = (loc, points) => {
  let minDist = Number.MAX_SAFE_INTEGER;
  let nearestIndex = 0;

  for (let i = 0; i < points.length; i++) {
    const point = points[i];
    const d = utils.distance(loc, point);

    if (d < minDist) {
      minDist = d;
      nearestIndex = i;
    }
  }
  return nearestIndex;
}
utils.styles = {
  car: { color: 'gray', text: '🚗' },
  fish: { color: 'red', text: '🐠' },
  house: { color: 'yellow', text: '🏠' },
  tree: { color: 'green', text: '🌳' },
  bicycle: { color: 'cyan', text: '🚲' },
  guitar: { color: 'blue', text: '🎸' },
  pencil: { color: 'magenta', text: '✏' },
  clock: { color: 'lightgray', text: '🕔' }
}
utils.styles["?"] = { color: 'red', text: '❓' };

utils.printProgress = (count, max) => {
  process.stdout.clearLine();
  process.stdout.cursorTo();
  const percent = utils.formatPercent(
    count / max
  );
  process.stdout.write(count + "/" + max +
    " (" + percent + ")"
  );
}

utils.groupBy = (objArray, key) => {
  const groups = {};
  for (let obj of objArray) {
    const val = obj[key];
    if (groups[val] == null) {
      groups[val] = [];
    }
    groups[val].push(obj);
  }
  return groups;
}

if (typeof module !== 'undefined') {
  module.exports = utils
}
