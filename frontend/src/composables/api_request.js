import axios from "axios";
import { useErrorStore } from "@/stores/error";

function requestErrorMsg(path) {
    return "Failed to connect to the server (" + path + ")";
}

function api_request(method, path, data = null, successCallback = null, errCallback = null) {
    const errorStore = useErrorStore();
    path = "/api/" + path;

    axios({
        method: method,
        url: path,
        data: data
    })
    .then(function (response) {
        if (response.data.err) {
            if (errCallback)
                errCallback();
            
            errorStore.msgs.push(response.data.err);
            return;
        }

        if (successCallback)
            successCallback(response.data);
    })
    .catch (function (err) {
        if (errCallback)
            errCallback();
        errorStore.msgs.push(requestErrorMsg(path));
    });
}

export function api_get(path, data = null, successCallback = null, errCallback = null) {
    api_request("get", path, data, successCallback, errCallback);
}

export function api_post(path, data = null, successCallback = null, errCallback = null) {
    api_request("post", path, data, successCallback, errCallback);
}