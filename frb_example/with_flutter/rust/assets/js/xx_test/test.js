import {thirdFunc, fourthFunc} from "./test2.js";

async function main() {
  console.log('Hello, World 2 from JS!');
   

    //globalThis.hello = `${first} ${second}`;
    console.log("input from test " + globalThis.input);
    globalThis.stmt = await second_func();
  }
  // mainFunc();
  // default_func();
  // sqlSelect();

  async function second_func(){
    console.log('Hello, World 4 from JS!');
    thirdFunc();
    const [first, second] = await Promise.all([
      Promise.resolve('hello'),
      Promise.resolve('world')
    ]);
    return `${first} ${second}`;
  }
  
  main().catch(e => console.error(e));