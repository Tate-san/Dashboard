import axios from "axios";

const axiosAPI = axios.create({
    baseURL: "http://localhost:8080/api/"
});

export const apiRequest = (method, url, body) => {
    const headers = {
        "Access-Control-Allow-Origin": true,
    };

    return axiosAPI({
        method,
        url,
        data: body,
        headers,
        withCredentials: true
    })
    .then(res => {
        return Promise.resolve(res.data);
    })
    .catch((e) => {
        if(e.response && e.response.data) return Promise.reject(e.response.data)
        return Promise.reject(e);
    });
};