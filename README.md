# Vexide Simulator Protocol

The Vexide Simulator Protocol enables communication between VEX robot simulators and user-facing frontends using a JSON-based protocol.

The code executor and frontend communicate over a stream in [newline-delimited JSON format](https://jsonlines.org/).

The backend sends `Event`s which represent a change in simulator state.
These are used by the frontend to correctly display the state of the simulated program.

The frontend sends `Command`s to the code executor to control the robot code environment, simulating changes in robot hardware (like controller input and LCD touch events) or competition phase.

The full protocol is documented at <https://internals.vexide.dev/simulators/protocol>.
