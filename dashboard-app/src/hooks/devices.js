import { apiRequest } from "./common";

export const device = {
    name: "",
    structure: [],
    topic: ""
}

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