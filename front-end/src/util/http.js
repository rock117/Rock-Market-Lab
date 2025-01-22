import axios from "axios";

const handleRequestFailed = error => {

}

// 超出 2xx reject
axios.interceptors.response.use(
	(response) => {
		if (response.status === "401") {
			unAuthorizedHandler();
			return {};
		} else {
			return response;
		}
	},
	(error) => {
		const status = error.response.status;
		console.error(`http stauts not ok: ${status}`)
		handleRequestFailed(error)
		return Promise.reject(error);
	}
);

axios.interceptors.request.use(function (config) {
    return config;
  }, function (error) {
    return Promise.reject(error);
  });

const get = (url, config) => {
	return axios.get(url, config);
};
const post = (url, data, config) => {
	return axios.post(url, data, config);
};
const put = (url, data, config) => {
	return axios.put(url, data, config);
};

const delete_ = (url, config) => {
	return axios.delete(url, config);
};
const upload = (url, file) => {
	const formData = new FormData();
	formData.append("file", file);
	return axios.post(url, formData, {
		headers: {
			"Content-Type": "multipart/form-data"
		}
	});
};

const request = (method, url, config) => {
	config = config || {};
	return axios({
		method: method,
		url: url,
		...config
	});
};

export { get, post, put, delete_, upload };
