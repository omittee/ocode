function generateTemplate(): HTMLTemplateElement {
  let template: HTMLTemplateElement | null = document.getElementById('ohcode-template') as HTMLTemplateElement;
  if (template instanceof HTMLTemplateElement) return template;
  template = document.createElement('template');
  template.setAttribute('id', 'ohcode-template');
  template.innerHTML = `<div class="box" part="box"><div class="scroll-box" part="scroll-box"></div></div>`
  document.body.appendChild(template);
  return template;
}

export class OhCodeEle extends HTMLElement {
  constructor() {
    super();
    const template = generateTemplate();
    const main = template.content.cloneNode(true);
    const style = document.createElement('style');
    style.textContent = `
    .box {
      overflow-y: auto;
      max-height: 40em;
    }
    .scroll-box {
      position: relative;
    }
    .item {
      position: absolute;
      white-space: nowrap;
    }
    .line-num {
      display: inline-block;
      text-align: right;
      padding-right: 1em;
      width: var(--line-num-width, auto);
      white-space: pre;
    }
    .delete {
      background-color: rgba(255, 0, 0, 0.5);
    }
    .add {
      background-color: rgba(0, 128, 0, 0.5)
    }`
    main.appendChild(style);
    const shadow = this.attachShadow({ mode: 'open' });
    shadow.appendChild(main);
  }
}