declare var self: Worker;
const diffAlgorithm = import('@omittee/ohcode-diff');

const { compute_diff } = await diffAlgorithm;
self.onmessage = (e) => {
  const { srcLine, tgtLine} = e.data;
  const diffRes = compute_diff(srcLine, tgtLine);
  self.postMessage(diffRes, [diffRes.buffer]);
}

export {}