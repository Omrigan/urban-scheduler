# Urban scheduler
[![Build Status](https://ci.urbanscheduler.ml/api/badges/Omrigan/urban-scheduler/status.svg)](https://ci.urbanscheduler.ml/Omrigan/urban-scheduler)


## Usage scenario
When someone wants to use the app, he creates a schedule for the day. The schedule may contain placeholders, specifying only category of the place desired to visit. 

Urban Scheduler is able to substitute event placeholder with actual place, optimizing total travel time.

### Supported cities:

- Moscow
- Helsinki

## Underlying technologies
The system is written in python, Rust and JavaScript. 

We use 
## External service providers

### HERE

HERE.com is used for several tasks inside the application:
 - One of the candidate's routers
 - Final schedule and route


### OpenStreetMap

OSRM is used for building routes between specific places. 

### Google Maps
Used for some places data.
### Moscow Open Data

## Algorithm

The system is able to solve the optimization problem called "POGTSP" - Partial-Ordered Generalized Travelling Salesman Problem.

The optimization is performed through mixed algorithm: branch&bound and dynamic programming.