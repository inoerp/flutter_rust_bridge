 function thirdFunc(){
  Deno.core.print("\n Hello run js! thirdFunc");
    console.log('\n Hello, World from thirdFunc using console.log');
    return 10;
  }

Deno.core.print("\n Hello run js outside!");


  function fourthFunc(){
    console.log('\n Hello, World from fourthFunc! using console.log');
  }

  export {
    thirdFunc,
    fourthFunc
  }

