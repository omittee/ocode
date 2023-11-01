import { defineConfig } from 'vite'
// import { nodeResolve } from '@rollup/plugin-node-resolve'
import commonjs from '@rollup/plugin-commonjs';
import wasm from 'vite-plugin-wasm'


export default defineConfig({
  build: {
    lib: {
      entry: './lib/main.ts',
      name: 'Ohcode-editor',
      fileName: 'ohcode-editor'
    }
  },
  // define: {
  //   __dirname: `'${path.join(path.resolve(), 'node_modules', 'ohcode-highlight.linux-x64-gnu')}'`
  // },
  plugins: [
    wasm(),
    commonjs({
    ignore: function (id) {
      console.log(id)
      return id.indexOf('linux-x64-gnu') > -1;
    },
  })]
})
