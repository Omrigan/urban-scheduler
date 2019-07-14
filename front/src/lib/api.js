import axios from "axios";
import {BACKEND_URL} from './settings'
import React from 'react';
import update from 'immutability-helper';


const fetchCallback = (setOptions, result) => {
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

    setOptions({
        categoriesList: categoriesList,
        categories: result.data,
        brands: brands
    });
};

export const OptionsContext = React.createContext({});
export const CenterContext = React.createContext([0, 0]);

export const getOptions = (setOptions) => {
    return axios.get(BACKEND_URL + '/get_params').then((result) => fetchCallback(setOptions, result));
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

        setState({cities: cityOptions2, citiesRaw: result.data});
        // setState((oldState) => (update(oldState, {cities: cityOptions2, citiesRaw: result.data, config: {city: "moscow"}})));
    });
};


export const postJob = (data, updateResult) => {
    axios.post(BACKEND_URL + '/predict', data)
        .catch((error) => updateResult(null, error.response.data))
        .then((response) => updateResult(response.data, null));
};