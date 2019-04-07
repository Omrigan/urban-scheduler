export const saveEventStates = (eventStates) => {
    localStorage.setItem('eventStates', JSON.stringify(eventStates))
};

export const loadEventStates = () => {
    let loadedState = localStorage.getItem('eventStates');

    if (loadedState) {
        loadedState = JSON.parse(loadedState);
        return loadedState;
    }
    return [];
};

export const saveResult = (eventStates) => {
    localStorage.setItem('result', JSON.stringify(eventStates))
};

export const loadResult = () => {
    let loadedState = localStorage.getItem('result');

    if (loadedState) {
        loadedState = JSON.parse(loadedState);
        return loadedState;
    }
    return null;
};
