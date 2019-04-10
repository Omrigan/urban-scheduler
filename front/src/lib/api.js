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

export const CenterContext = React.createContext([0, 0]);


export const getOptions = (setState) => {
    return axios.get(BACKEND_URL + '/get_params').then((result) => fetchCallback(setState, result));
};


export const getCities = (setState) => {
    axios.get(BACKEND_URL + '/cities').then((result) => {
        const cityOptions = Object.keys(result.data);
        const cityOptions2 = cityOptions.map((city) =>
            ({
                text: city,
                key: city,
                value: city
            }));
        console.log(cityOptions2);

        setState(() => ({cities: cityOptions2, citiesRaw: result.data}));
    });
};


export const postJob = (data, updateResult) => {
    axios.post(BACKEND_URL + '/predict', data)
        .catch((error) => updateResult(null, error.response.data))
        .then((response) => updateResult(response.data, null));
};