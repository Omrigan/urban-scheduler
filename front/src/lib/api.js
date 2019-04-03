import axios from "axios";
import {BACKEND_URL} from './settings'
import React from 'react';

const fetchCallback = (setState, result) => {
    const categoriesList = result.data.map((x, i) =>
        ({
            text: x.name,
            key: x.name,
            value: x.name
        }));
    const brands = result.data.reduce((obj, cat) => {
        obj[cat.name] = cat.brands.map(brand => ({
            text: brand,
            key: brand,
            value: brand
        }));
        return obj;
    }, {});

    setState(
        {
            categoriesList: categoriesList,
            categories: result.data,
            brands: brands
        });
};

export const OptionsContext = React.createContext({});

export const getOptions = (setState) => {
    return axios.get(BACKEND_URL + '/get_params').then((result) => fetchCallback(setState, result));
};


export const postJob = (data, callbackResult) => {
    axios.post(BACKEND_URL + '/predict', data).then((response) => callbackResult(response.data))
};