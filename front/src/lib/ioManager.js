import {safeDump, safeLoad} from "js-yaml";

export const saveProblem = (problem) => {
    localStorage.setItem("problem", JSON.stringify(problem))
};

export const loadProblem = () => {
    let loadedState = localStorage.getItem("problem");
    if (loadedState) {
        loadedState = JSON.parse(loadedState);
        return loadedState;
    }
};

export const exportProblem = (problem) => {
    const element = document.createElement("a");
    const file = new Blob([safeDump(problem)], {type: 'plain/text'});
    element.href = URL.createObjectURL(file);
    element.download = "problem.yaml";
    document.body.appendChild(element); // Required for this to work in FireFox
    element.click();
};

export const importProblem = (e) => {
    let file = e.target.files[0];
    if (!file) {
        return;
    }
    let reader = new FileReader();
    reader.onload = (e) => {
        let problem = safeLoad(e.target.result);
        this.setProblem(problem);
    };
    reader.readAsText(file);
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
