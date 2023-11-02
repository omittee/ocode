import { OhCode } from "../lib/main";

const app = document.getElementById("app");

const origin = Array(3000).fill(0).map((i) => String.fromCharCode(Math.floor(Math.random() * ('Z'.charCodeAt(0) - '0'.charCodeAt(0))) + '0'.charCodeAt(0)).repeat(Math.random() * 200 + 10));

const modified = origin.reduce((res: string[], x, i) => {
  const t = Math.random();
  if (t < 0.25) return res;
  else if (t < 0.5) {
    res.push(String.fromCharCode(Math.floor(Math.random() * ('Z'.charCodeAt(0) - '0'.charCodeAt(0))) + '0'.charCodeAt(0)).repeat(Math.random() * 200 + 10))
    res.push(x)
  }
  else if (t < 0.75) {
    res.push(String.fromCharCode(Math.floor(Math.random() * ('Z'.charCodeAt(0) - '0'.charCodeAt(0))) + '0'.charCodeAt(0)).repeat(Math.random() * 200 + 10))
  }
  else res.push(x);
  return res;
}, [])

console.time("render")
new OhCode({
  origin,
  modified,
  parent: app!
})
console.timeEnd("render")