import {thirdFunc, fourthFunc} from "./test2.js";

function triple(a) {
  console.log("triple(" + a + ")");
  return 3 * a;
}

function extract(obj) {
  console.log("extract(" + obj + ")");

  return {
      new_text: obj.text + ".",
      new_num: triple(obj.num)
  };
}

function mainFunc2(obj) {
  console.log("\n In main_func2");
}

async function mainFunc() {
  console.log('\n Hello, World 2 from JS!');
   

    //globalThis.hello = `${first} ${second}`;
    console.log("input from test " + globalThis.input);
    globalThis.stmt = await second_func();
  }

  mainFunc2();
  mainFunc();
  default_func();
  sqlSelect();

  async function second_func(){
    console.log('Hello, World 4 from JS!');
    thirdFunc();
    const [first, second] = await Promise.all([
      Promise.resolve('hello'),
      Promise.resolve('world')
    ]);
    return `${first} ${second}`;
  }
  
  mainFunc().catch(e => console.error(e));

  export {
    mainFunc2,
    mainFunc
  }