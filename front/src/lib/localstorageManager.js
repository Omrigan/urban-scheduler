export const saveEventStates = (eventStates) => {
    localStorage.setItem('eventStates', JSON.stringify(eventStates))
};

export const loadEventStates = () => {
    let loadedState = localStorage.getItem('eventStates');

    if (loadedState) {
        loadedState = JSON.parse(loadedState);
        return loadedState;
    }
    return null;
};