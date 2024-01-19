function validate() {
  console.log("printing input");
  console.log(data);
  if (!Array.isArray(data)) {
    return "false";
  }
  let firstHeader = data[0]['header'][0];
  if (firstHeader['last_insert_id'] > 0) {
    return "true";
  }

  return "false";
}
validate();


function validate(dataStr) {
  try {
    const data = JSON.parse(dataStr);
    if (!Array.isArray(data['header'])) {
      return false;
    }
    let firstHeader = data.header[0];
    if (firstHeader['last_insert_id'] > 0) {
      return true;
    }
  } catch (error) {
    return false;
  }

  return false;
}
validate(data);

const data =
 '{    "header": [         {            "last_insert_id": "385"         },        {            "rows_affected": "1"        }    ],    "lines": [        {            "last_insert_id": "470"        },        {            "rows_affected": "9"        }    ]}';

  let result = validate(data);

  console.log("result is " + result);

