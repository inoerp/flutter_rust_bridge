function global() {
    consoleLog("In global function");
  }
  
  function getData(inputData) {
    if (typeof inputData === "string") {
      return JSON.parse(inputData);
    } else {
      return inputData;
    }
  }
  
  function getDataFromSql(selectSql) {
    request = {
      sql: selectSql,
      dbType: "MySQL",
      connName: "Inoerp"
    };
  
    let response = sqlSelect(request);
  
    return response["data"];
  }
  
  function updateDataWithSql(updateSql) {
    request = {
      sql: updateSql,
      dbType: "MySQL",
      connName: "Inoerp"
    };
  
    let response = sqlUpdate(request);
  
    return response["data"];
  }
  
  function insertDataWithSql(insertSql) {
    request = {
      sql: insertSql,
      dbType: "MySQL",
      connName: "Inoerp"
    };
  
    let response = sqlInsert(request);
  
    return response["data"];
  }
  
  function deleteDataWithSql(deleteSql) {
    request = {
      sql: deleteSql,
      dbType: "MySQL",
      connName: "Inoerp"
    };
  
    let response = sqlDelete(request);
  
    return response["data"];
  }
  
  function getFlatObject(obj, flatObject) {
    if (Object.keys(obj).length > 0) {
      let allKeys = Object.keys(obj);
      for (let i = 0; i < allKeys.length; i++) {
        const k = allKeys[i];
        if (k != null) {
          flatObject[k] = obj[k];
          if (typeof obj[k] === "object" && Object.keys(obj[k]).length > 0) {
            getFlatObject(obj[k], flatObject);
          }
        }
      }
    }
  }
  
  function getFormattedDate(date) {
    if (date != null) {
      try {
        let isoDate = new Date(date).toISOString();
        if (isoDate !== "Invalid Date") {
          return isoDate.split("T")[0];
        } else {
          consoleLog("\nInvalid Date 1 : " + date);
          return null;
        }
      } catch (error) {
        consoleLog("\nInvalid Date 2 : " + date);
        return null;
      }
    } else {
      return null;
    }
  }
  
  function getMessageFromSqlResponse(sqlResponse) {
    if (
      sqlResponse["data"] != undefined &&
      Array.isArray(sqlResponse["data"]) &&
      sqlResponse["data"].length > 0
    ) {
      return sqlResponse["data"][0]["message"];
    } else {
      return JSON.stringify(sqlResponse["data"]);
    }
  }
  function getStatusFromAction(action) {
    var status = null;
    // if (action == "cancel") {
    //   status = "CANCELLED";
    // } else if (action == "confirm") {
    //   status = "CONFIRMED";
    // } else if (action == "close") {
    //   status = "CLOSED";
    // } else if (action == "reopen" || action == "open") {
    //   status = "DRAFT";
    // } else if (action == "hold") {
    //   status = "ON_HOLD";
    // }
  
    switch (action) {
      case "cancel":
        status = "CANCELLED";
        break;
      case "confirm":
        status = "CONFIRMED";
        break;
      case "close":
        status = "CLOSED";
        break;
      case "reopen":
      case "open":
        status = "DRAFT";
        break;
      case "hold":
        status = "ON_HOLD";
        break;
      case "active":
        status = "ACTIVE";
        break;
      case "post":
        status = "POSTED";
        break;
      case "approve":
        status = "approved";
        break;
      case "reject":
        status = "rejected";
        break;
      case "need_more_info":
        status = "need_more_info";
    }
    return status;
  }
  
  function printNestedObject(obj) {
    if (typeof obj === "string") {
      consoleLog(obj);
    } else if (Object.keys(obj).length > 0) {
      let allKeys = Object.keys(obj);
      for (let i = 0; i < allKeys.length; i++) {
        const k = allKeys[i];
        if (k != null) {
          consoleLog(k + " : " + obj[k]);
          if (typeof obj[k] === "object" && Object.keys(obj[k]).length > 0) {
            printNestedObject(obj[k]);
          }
        }
      }
    }
  }
  
  function isNull(str) {
    return !isNotNull(str);
  }
  
  function isNotNull(str) {
    if (
      str != undefined &&
      str != null &&
      str != "" &&
      str != "null" &&
      str.length > 0
    ) {
      return true;
    } else {
      return false;
    }
  }
  
  function updateAsPerDocStatus(data) {
    var docStatus = data["docStatus"];
    var readOnly = true;
    var entityObjectReadOnly = false;
  
    if (docStatus == "DRAFT") {
      readOnly = false;
    } else if (
      docStatus == "CANCELLED" ||
      docStatus == "CLOSED" ||
      docStatus == "retired" ||
      docStatus == "POSTED"
    ) {
      entityObjectReadOnly = true;
    }
    var entityObject = { readonly: entityObjectReadOnly };
    var dataDefinition = { entityObject: entityObject };
    let allKeys = Object.keys(data);
    for (let i = 0; i < allKeys.length; i++) {
      const k = allKeys[i];
      if (k != null) {
        var dd = {};
        if (k == "description" || k == "orderSourceType") {
          dd["readonly"] = false;
        } else {
          dd["readonly"] = readOnly;
        }
        dataDefinition[k] = dd;
      }
    }
    data["dataDefinition"] = dataDefinition;
    return data;
  }
  
  function validationForAfterGet(inputData) {
    if (inputData != null) {
    } else {
      return inputData;
    }
    if (!inputData["pathParamValues"]) {
      return inputData["data"];
    }
  
    var data = inputData["data"];
    var isArray = data.constructor === Array ? true : false;
  
    if (isArray) {
      var retData = [];
      for (let index = 0; index < data.length; index++) {
        var element = data[index];
        updateAsPerDocStatus(element);
        retData.push(element);
      }
      return retData;
    } else {
      updateAsPerDocStatus(data);
      return data;
    }
  }
  