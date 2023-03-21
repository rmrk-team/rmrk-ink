import { ReturnNumber } from "@727-ventures/typechain-types";
import { expect } from "chai";

// Helper function to parse Events
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const emit = (result: { events?: any }, name: string, args: any): void => {
  const event = result.events.find(
    (event: { name: string }) => event.name === name
  );
  for (const key of Object.keys(event.args)) {
    if (event.args[key] instanceof ReturnNumber) {
      event.args[key] = event.args[key].toNumber();
    }
  }
  expect(event).eql({ name, args });
}