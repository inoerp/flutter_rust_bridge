function beforeGet(){
    console.log("In before patch");
    let obj = {
      rd_proceed_status : true,
      rd_proceed_message : "100" ,
    };
    return obj;
}