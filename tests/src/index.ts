import { Config, InstallAgentsHapps, Orchestrator } from "@holochain/tryorama";
import Base64 from "js-base64";
import path from "path";

const conductorConfig = Config.gen();

// Construct proper paths for your DNAs
const recurrentEventsDna = path.join(__dirname, "../../workdir/dna/recurrent-events-test.dna");

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [recurrentEventsDna],
  ],
  [
    // happ 0
    [recurrentEventsDna],
  ],
];

const sleep = (ms) =>
  new Promise((resolve) => setTimeout(() => resolve(null), ms));

function serializeHash(hash) {
  return `u${Base64.fromUint8Array(hash, true)}`;
}

const zomeName = 'recurrent_events';

let orchestrator = new Orchestrator();

orchestrator.registerScenario("create a recurrentEvent and get it", async (s, t) => {
  const [alice, bob] = await s.players([conductorConfig]);

  // install your happs into the coductors and destructuring the returned happ data using the same
  // array structure as you created in your installation array.
  const [[alice_recurrent_events], [bob_recurrent_events]] = await alice.installAgentsHapps(
    installation
  );


  let alicePubkeyB64 = serializeHash(alice_recurrent_events.agent);
  let bobPubKeyB64 = serializeHash(bob_recurrent_events.agent);

  let myRecurrentEvent = await alice_recurrent_events.cells[0].call(
    zomeName,
    "get_my_recurrent_event",
    null
  );
  t.notOk(myRecurrentEvent);

  let recurrentEventHash = await alice_recurrent_events.cells[0].call(
    zomeName,
    "create_recurrent_event",
    {
      nickname: "alice",
      fields: {
        avatar: "aliceavatar",
      },
    }
  );
  t.ok(recurrentEventHash);

  await sleep(500);

  // set nickname as alice to make sure bob's is not getting deleted
  // with alice's update
  recurrentEventHash = await bob_recurrent_events.cells[0].call(zomeName, "create_recurrent_event", {
    nickname: "alice_bob",
    fields: {
      avatar: "bobboavatar",
    },
  });
  t.ok(recurrentEventHash);

  await sleep(5000);

  recurrentEventHash = await alice_recurrent_events.cells[0].call(
    zomeName,
    "update_recurrent_event",
    {
      nickname: "alice2",
      fields: {
        avatar: "aliceavatar2",
        update: "somenewfield",
      },
    }
  );
  t.ok(recurrentEventHash);

  myRecurrentEvent = await alice_recurrent_events.cells[0].call(
    zomeName,
    "get_my_recurrent_event",
    null
  );
  t.ok(myRecurrentEvent.agentPubKey);
  t.equal(myRecurrentEvent.recurrentEvent.nickname, "alice2");

  let allrecurrentEvents = await bob_recurrent_events.cells[0].call(
    zomeName,
    "get_all_recurrent_events",
    null
  );
  t.equal(allrecurrentEvents.length, 2);

  let multipleRecurrentEvents = await bob_recurrent_events.cells[0].call(
    zomeName,
    "get_agents_recurrent_event",
    [alicePubkeyB64, bobPubKeyB64]
  );
  t.equal(multipleRecurrentEvents.length, 2);

  let recurrentEvents = await bob_recurrent_events.cells[0].call(
    zomeName,
    "search_recurrent_events",
    {
      nicknamePrefix: "sdf",
    }
  );
  t.equal(recurrentEvents.length, 0);

  recurrentEvents = await bob_recurrent_events.cells[0].call(zomeName, "search_recurrent_events", {
    nicknamePrefix: "alic",
  });
  t.equal(recurrentEvents.length, 2);
  t.ok(recurrentEvents[0].agentPubKey);
  t.equal(recurrentEvents[1].recurrentEvent.nickname, "alice2");

  recurrentEvents = await bob_recurrent_events.cells[0].call(zomeName, "search_recurrent_events", {
    nicknamePrefix: "ali",
  });
  t.equal(recurrentEvents.length, 2);
  t.ok(recurrentEvents[0].agentPubKey);
  t.equal(recurrentEvents[1].recurrentEvent.nickname, "alice2");
  t.equal(recurrentEvents[1].recurrentEvent.fields.avatar, "aliceavatar2");

  recurrentEvents = await bob_recurrent_events.cells[0].call(zomeName, "search_recurrent_events", {
    nicknamePrefix: "alice",
  });
  t.equal(recurrentEvents.length, 2);
  t.ok(recurrentEvents[1].agentPubKey);
  t.equal(recurrentEvents[1].recurrentEvent.nickname, "alice2");

  recurrentEvents = await bob_recurrent_events.cells[0].call(zomeName, "search_recurrent_events", {
    nicknamePrefix: "alice_",
  });
  t.equal(recurrentEvents.length, 2);
  t.ok(recurrentEvents[0].agentPubKey);
  t.equal(recurrentEvents[0].recurrentEvent.nickname, "alice_bob");
  t.equal(recurrentEvents[0].recurrentEvent.fields.avatar, "bobboavatar");
});

orchestrator.run();
orchestrator = new Orchestrator();

orchestrator.registerScenario(
  "create a recurrentEvent with upper case and search it with lower case",
  async (s, t) => {
    const [alice, bob] = await s.players([conductorConfig]);

    // install your happs into the coductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_recurrent_events], [bob_recurrent_events]] = await alice.installAgentsHapps(
      installation
    );

    let recurrentEventHash = await alice_recurrent_events.cells[0].call(
      zomeName,
      "create_recurrent_event",
      {
        nickname: "ALIce",
        fields: {
          avatar: "aliceavatar",
        },
      }
    );
    t.ok(recurrentEventHash);
    await sleep(5000);

    let recurrentEvents = await bob_recurrent_events.cells[0].call(
      zomeName,
      "search_recurrent_events",
      {
        nicknamePrefix: "ali",
      }
    );
    t.equal(recurrentEvents.length, 1);
    t.ok(recurrentEvents[0].agentPubKey);
    t.equal(recurrentEvents[0].recurrentEvent.nickname, "ALIce");

    recurrentEvents = await bob_recurrent_events.cells[0].call(zomeName, "search_recurrent_events", {
      nicknamePrefix: "aLI",
    });
    t.equal(recurrentEvents.length, 1);
    t.ok(recurrentEvents[0].agentPubKey);
    t.equal(recurrentEvents[0].recurrentEvent.nickname, "ALIce");

    recurrentEvents = await bob_recurrent_events.cells[0].call(zomeName, "search_recurrent_events", {
      nicknamePrefix: "AlI",
    });
    t.equal(recurrentEvents.length, 1);
    t.ok(recurrentEvents[0].agentPubKey);
    t.equal(recurrentEvents[0].recurrentEvent.nickname, "ALIce");

    recurrentEvents = await bob_recurrent_events.cells[0].call(zomeName, "search_recurrent_events", {
      nicknamePrefix: "ALI",
    });
    t.equal(recurrentEvents.length, 1);
    t.ok(recurrentEvents[0].agentPubKey);
    t.equal(recurrentEvents[0].recurrentEvent.nickname, "ALIce");
  }
);

orchestrator.run();
