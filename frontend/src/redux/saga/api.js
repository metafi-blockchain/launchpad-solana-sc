import axios from "axios";
import {
  BASE_URL,
} from "../../constants";

const instance = (headers) => {
  let returnValue = axios.create();

  returnValue.interceptors.request.use(
    async (config) => {
      return config;
    },
    (error) => {
      console.log(error);
      return Promise.reject(error);
    }
  );
  returnValue.interceptors.response.use(
    (response) => {
      return response;
    },
    async (error) => {
      const originalRequest = error.response.config;
      return Promise.reject(error);
    }
  );
  return returnValue;
};

export const apiCall = (
  method,
  url,
  data,
  headers,
  useBody,
) => {

  const config = {
    method,
    url,
    baseURL: BASE_URL,
  };
  if (method === "GET") {
    config.params = data;
  } else {
    config.data = data;
  }
  return instance(headers)(config).then((response) => {
    return response;
  });
};
export const apiCallErrorHanding = async (
  method,
  url,
  data,
  options,
) => {
  let result;
  try {
    const response = await instance()({
      method,
      url,
      data,
      ...options,
    });
    result = response.data;
    return result;
  } catch (error) {
    result = error.response || error;
    return result;
  }
};
