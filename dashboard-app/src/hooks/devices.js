import { apiRequest } from "./common";

export const device = {
    name: "",
    topic: "",
    structure: []
};

export const structureValue = {
    devicestructure_id: 0,
    real_name: "",
    alias_name: "",
    data_type: ""
};

export const structureDataTypes = [
    {
        value: "int",
        name: "Integer"
    },
    {
        value: "float",
        name: "Float"
    }
];

export async function getDeviceList() {
    return apiRequest("get", `/device/list`, null)
    .then((data) => {
        return Promise.resolve(data);
    })
    .catch((e) => {
        return Promise.reject(e);
    })
}

export async function addDevice(new_device) {
    return apiRequest("post", "/device", new_device)
    .then((data) => {
        return Promise.resolve(data);
    })
    .catch((e) => {
        return Promise.reject(e);
    })
}

export async function deleteDevice(device_id) {
    return apiRequest("delete", `/device/${device_id}`, null);
}

export async function getDevice(device_id) {
    return apiRequest("get", `/device/${device_id}`, null);
}

export async function updateDevice(device_id, device_object)
{
    return apiRequest("patch", `/device/${device_id}`, device_object);
}