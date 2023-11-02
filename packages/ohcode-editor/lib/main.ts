import { OhCodeOptions, DiffType } from "./types";
import { OhCodeEle } from "./component/ohcode";
import DiffWorker from './worker?worker'

customElements.define('oh-code', OhCodeEle);

export class OhCode {
  #option: OhCodeOptions;
  #map: Map<string, number> = new Map();

  #box: HTMLElement;
  #scollBox: HTMLElement;
  #lines: HTMLElement[] = [];

  #lineHeight: number = 20;

  #worker: Worker

  constructor(option: OhCodeOptions) {
    const ohcode = document.createElement('oh-code');
    option.parent.appendChild(ohcode);
    this.#option = option;
    const srcLine = Uint32Array.from(option.origin.map(s => this.#getHash(s)));
    const tgtLine = Uint32Array.from(option.modified.map(s => this.#getHash(s)));
    this.#box = ohcode.shadowRoot!.querySelector(".box")!;
    this.#scollBox = ohcode.shadowRoot!.querySelector(".scroll-box")!;

    this.#worker = new DiffWorker();
    this.#worker.onmessage = ({ data }) => {
      this.#lines = this.#geterateContent(data);
      this.#lineHeight = this.#scollBox.firstElementChild?.getBoundingClientRect().height ?? 20;
      this.#adjustStyle();
      this.#initContent();
      this.#useScroll();
    }
    this.#worker.postMessage({ srcLine, tgtLine }, [srcLine.buffer, tgtLine.buffer]);
  }

  #getHash(s: string): number {
    if (this.#map.has(s)) return this.#map.get(s)!;
    const len = this.#map.size;
    this.#map.set(s, len);
    return len
  }

  #generateLine(str: string, type: DiffType, offset: number): HTMLElement {
    const line = document.createElement("div");
    line.classList.add("item");
    line.style.top = offset * 1.25 + 'em';

    const lineNum = document.createElement("span");
    lineNum.classList.add("line-num");
    lineNum.textContent = `${offset + 1}`;
    if (type === DiffType.Add) {
      line.classList.add("add");
      lineNum.textContent += '+'
    }
    else if (type === DiffType.Delete) {
      line.classList.add("delete");
      lineNum.textContent += '-'
    }
    else lineNum.textContent += ' '
    line.append(lineNum, new Text(str));
    return line;
  }

  #geterateContent(diffRes: Uint32Array): HTMLElement[] {
    const res = [];
    for (let i = 0, si = 0, ti = 0, cnt = 0; si < this.#option.origin.length || ti < this.#option.modified.length;) {
      if (si !== diffRes[i] && ti !== diffRes[i + 2]) {
        res.push(this.#generateLine(this.#option.origin[si], DiffType.Normal, cnt++));
        si++, ti++;
        continue;
      }
      if (si === diffRes[i])
        while (si < diffRes[i + 1]) res.push(this.#generateLine(this.#option.origin[si++], DiffType.Delete, cnt++));
      if (ti === diffRes[i + 2])
        while (ti < diffRes[i + 3]) res.push(this.#generateLine(this.#option.modified[ti++], DiffType.Add, cnt++));
      i += 4;
    }
    return res;
  }

  #initContent() {
    const frag = document.createDocumentFragment()
    for (let i = 0; i < Math.min(this.#lines.length, 50); i++)
      frag.appendChild(this.#lines[i]);
    this.#scollBox.appendChild(frag);
  }

  #adjustStyle() {
    this.#scollBox.style.height = this.#lines.length * 1.25 + 'em';

    // compute the lineNum width
    let w = 0;
    for (let t = Math.max(this.#map.size, 1); t; t = Math.floor(t / 10)) w++;
    this.#scollBox.style.setProperty("--line-num-width", `${w}em`);
  }

  #useScroll() {
    let scrollTop = 0;
    const callback = () => {
      const mid = Math.floor(scrollTop / this.#lineHeight);
      const [start, end] = [Math.max(0, mid - 10), Math.min(this.#lines.length - 1, mid + 40)];
      const [startOffset, endOffset] = [start * this.#lineHeight, end * this.#lineHeight];
      for (const x of this.#scollBox.children) {
        const t = (x as HTMLElement).offsetTop;
        if (t < startOffset || t > endOffset)
          this.#scollBox.removeChild(x);
      }
      const frag = document.createDocumentFragment();
      for (let i = start; i <= end; i++) {
        if (!this.#scollBox.contains(this.#lines[i])) frag.appendChild(this.#lines[i]);
      }
      this.#scollBox.appendChild(frag);
    }
    this.#box.addEventListener("scroll", e => {
      scrollTop = (e.currentTarget as HTMLElement).scrollTop;
      requestAnimationFrame(callback)
    })
  }
}
