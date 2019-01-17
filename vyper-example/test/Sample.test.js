const Sample = artifacts.require("Sample");

contract("Sample", () => {

  it("should store the value.", async () => {
    const storage = await Sample.deployed();

    const value = 10;

    await storage.set(value);

    const storedData = await storage.get();

    assert.equal(storedData, value, `The value ${value} was not stored.`);
  });

  it("should store the value to map", async () => {
    const storage = await Sample.deployed();

    const key = 5;
    const value = 10;

    await storage.mapSet(key, value);

    const storedData = await storage.mapGet(key);

    assert.equal(storedData, value, `The value ${value} was not stored.`);
  });

});
