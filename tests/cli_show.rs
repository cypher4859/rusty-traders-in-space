use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};
use strip_ansi_escapes::strip;

#[tokio::test]
async fn show_factions_as_json() -> anyhow::Result<()> {
    // 1) Start mock API
    let server = MockServer::start().await;

    // GET /factions (fallback list for name-based match)
    let list = serde_json::json!({
        "data": [{
            "symbol": "CULT",
            "name": "Cult of the Machine",
            "description": "…",
            "headquarters": "X1-TEST-123",
            "traits": [],
            "isRecruiting": true
        }],
        "meta": { "total": 1, "page": 1, "limit": 20 }
    });
    let mock = Mock::given(method("GET"))
        .and(path("v2/factions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list));

    server.register(mock).await;

    // 2) Run your binary (assumes you read base URL from an env var)
    // e.g., your HTTP client looks at SPACETRADERS_BASE_URL
    let mut cmd = Command::cargo_bin("spacetraders")?;
    cmd.env("SPACETRADERS_API_BASE_URL", format!("{}/v2", server.uri()))
       .env("SPACETRADERS_OUTPUT_MODE", "Json")          // optional, for stable output
       .args(["show", "factions"]);


    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Cult of the Machine")
            .and(predicate::str::contains("Cult"))
        );

    Ok(())
}

#[tokio::test]
async fn show_location_as_json() -> anyhow::Result<()> {
    let server = MockServer::start().await;

    let list = serde_json::json!({
        "data": {
            "symbol": "X1-SR77-A1",
            "type": "PLANET",
            "systemSymbol": "X1-SR77",
            "x": 9,
            "y": 24,
            "orbitals": [
                {
                    "symbol": "X1-SR77-A2"
                },
                {
                    "symbol": "X1-SR77-A3"
                },
                {
                    "symbol": "X1-SR77-A4"
                }
            ],
            "faction": {
                "symbol": "CULT"
            },
            "traits": [
                {
                    "symbol": "ROCKY",
                    "name": "Rocky",
                    "description": "A world with a rugged, rocky landscape, rich in minerals and other resources, providing a variety of opportunities for mining, research, and exploration."
                },
                {
                    "symbol": "OUTPOST",
                    "name": "Outpost",
                    "description": "A small, remote settlement providing essential services and a safe haven for travelers passing through."
                },
                {
                    "symbol": "METHANE_POOLS",
                    "name": "Methane Pools",
                    "description": "Large reservoirs of methane gas, used for fuel and in various industrial processes such as the production of hydrocarbons."
                },
                {
                    "symbol": "SALT_FLATS",
                    "name": "Salt Flats",
                    "description": "Expansive, barren plains covered in a thick layer of salt, offering unique opportunities for resource extraction, scientific research, and other activities."
                },
                {
                    "symbol": "THIN_ATMOSPHERE",
                    "name": "Thin Atmosphere",
                    "description": "A location with a sparse atmosphere, making it difficult to support life without specialized life-support systems."
                },
                {
                    "symbol": "MARKETPLACE",
                    "name": "Marketplace",
                    "description": "A thriving center of commerce where traders from across the galaxy gather to buy, sell, and exchange goods."
                }
            ],
            "modifiers": [],
            "chart": {
                "waypointSymbol": "X1-SR77-A1",
                "submittedBy": "CULT",
                "submittedOn": "2025-08-17T13:01:22.422Z"
            },
            "isUnderConstruction": false
        }
    });

    let mock = Mock::given(method("GET"))
        .and(path("v2/systems/X1-SR77/waypoints/X1-SR77-A1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list));

    server.register(mock).await;

    // 2) Run your binary (assumes you read base URL from an env var)
    // e.g., your HTTP client looks at SPACETRADERS_BASE_URL
    let mut cmd = Command::cargo_bin("spacetraders")?;
    cmd.env("SPACETRADERS_API_BASE_URL", format!("{}/v2", server.uri()))
       .env("SPACETRADERS_OUTPUT_MODE", "Json")          // optional, for stable output
       .args(["show", "location", "-l", "X1-SR77-A1"]);


    let assert = cmd.assert()
        .success();

    let raw = &assert.get_output().stdout;

    let clean = String::from_utf8(strip(raw)).unwrap();

    // 2) keep only the JSON block (from first '{' to last '}')
    let start = clean.find('{').expect("no '{' in stdout");
    let end   = clean.rfind('}').expect("no '}' in stdout");
    let json  = &clean[start..=end];

    // 3) parse + assert
    let v: Value = serde_json::from_str(json).expect("stdout wasn't valid JSON");
    let data = &v["data"];
    assert_eq!(data["symbol"], "X1-SR77-A1");
    assert_eq!(data["type"],   "PLANET");
    assert_eq!(data["systemSymbol"], "X1-SR77");
    assert_eq!(data["faction"]["symbol"], "CULT");

    Ok(())
}

#[tokio::test]
async fn show_market_as_json() -> anyhow::Result<()> {
    let server = MockServer::start().await;

    let list = serde_json::json!({
        "data": {
            "symbol": "X1-SR77-A1",
            "exports": [],
            "imports": [
                {
                    "symbol": "FOOD",
                    "name": "Galactic Cuisine",
                    "description": "A diverse range of foods from different planets, including fresh produce, meats, and prepared meals."
                },
                {
                    "symbol": "MEDICINE",
                    "name": "Medicine",
                    "description": "Medical products, including drugs, treatments, and medical equipment."
                },
                {
                    "symbol": "CLOTHING",
                    "name": "Clothing",
                    "description": "A wide range of clothing and fashion items, including garments, accessories, and textiles."
                },
                {
                    "symbol": "EQUIPMENT",
                    "name": "Equipment",
                    "description": "Tools and equipment used in various industries and applications."
                },
                {
                    "symbol": "JEWELRY",
                    "name": "Jewelry",
                    "description": "Exquisite and valuable pieces of jewelry made from rare materials and precious gems."
                },
                {
                    "symbol": "VIRAL_AGENTS",
                    "name": "Viral Agents",
                    "description": "Viruses or other biological agents that are used for various purposes, such as medical treatments, weapons, or environmental remediation."
                }
            ],
            "exchange": [
                {
                    "symbol": "FUEL",
                    "name": "Fuel",
                    "description": "High-energy fuel used in spacecraft propulsion systems to enable long-distance space travel."
                }
            ],
            "transactions": null,
            "tradeGoods": null
        }
    });

    let mock = Mock::given(method("GET"))
        .and(path("v2/systems/X1-SR77/waypoints/X1-SR77-A1/market"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list));

    server.register(mock).await;

    // 2) Run your binary (assumes you read base URL from an env var)
    // e.g., your HTTP client looks at SPACETRADERS_BASE_URL
    let mut cmd = Command::cargo_bin("spacetraders")?;
    cmd.env("SPACETRADERS_API_BASE_URL", format!("{}/v2", server.uri()))
       .env("SPACETRADERS_OUTPUT_MODE", "Json")          // optional, for stable output
       .args(["show", "market", "-w", "X1-SR77-A1"]);

    let assert = cmd.assert()
        .success();

    let raw = &assert.get_output().stdout;

    let clean = String::from_utf8(strip(raw)).unwrap();

    // 2) keep only the JSON block (from first '{' to last '}')
    let start = clean.find('{').expect("no '{' in stdout");
    let end   = clean.rfind('}').expect("no '}' in stdout");
    let json  = &clean[start..=end];

    // 3) parse + assert
    let v: Value = serde_json::from_str(json).expect("stdout wasn't valid JSON");
    let data = &v["data"];
    assert_eq!(data["symbol"], "X1-SR77-A1");
    assert_eq!(data["imports"][0]["symbol"], "FOOD");
    assert_eq!(data["imports"][1]["symbol"], "MEDICINE");
    assert_eq!(data["exchange"][0]["symbol"], "FUEL");

    Ok(())
}

#[tokio::test]
async fn show_cargo_as_json() -> anyhow::Result<()> {
    let server = MockServer::start().await;

    let list = serde_json::json!({
        "data": {
            "capacity": 40,
            "units": 1,
            "inventory": [
                {
                    "symbol": "NANOBOTS",
                    "name": "Nanobots",
                    "description": "Tiny robots at the nanoscale, used in various applications, such as medical treatments, manufacturing, and environmental cleanup.",
                    "units": 1
                }
            ]
        }
    });

    let mock = Mock::given(method("GET"))
        .and(path("v2/my/ships/DEVILELK666-1/cargo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&list));

    server.register(mock).await;

    // 2) Run your binary (assumes you read base URL from an env var)
    // e.g., your HTTP client looks at SPACETRADERS_BASE_URL
    let mut cmd = Command::cargo_bin("spacetraders")?;
    cmd.env("SPACETRADERS_API_BASE_URL", format!("{}/v2", server.uri()))
       .env("SPACETRADERS_OUTPUT_MODE", "Json")          // optional, for stable output
       .args(["show", "cargo", "-s", "DEVILELK666-1"]);

    let assert = cmd.assert()
        .success();

    let raw = &assert.get_output().stdout;

    let clean = String::from_utf8(strip(raw)).unwrap();

    let start = clean.find('{').expect("no '{' in stdout");
    let end   = clean.rfind('}').expect("no '}' in stdout");
    let json  = &clean[start..=end];

    // 3) parse + assert
    let v: Value = serde_json::from_str(json).expect("stdout wasn't valid JSON");
    let data = &v["data"];
    assert_eq!(data["capacity"], 40);
    assert_eq!(data["units"], 1);
    assert_eq!(data["inventory"][0]["symbol"], "NANOBOTS");
    assert_eq!(data["inventory"][0]["name"], "Nanobots");
    assert_eq!(data["inventory"][0]["units"], 1);

    Ok(())
}

async fn show_systems_as_json() -> anyhow::Result<()> {
    todo!();
}

async fn show_system_as_json() -> anyhow::Result<()> {
    todo!();
}

async fn show_waypoints_as_json() -> anyhow::Result<()> {
    todo!();
}

async fn show_waypoint_as_json() -> anyhow::Result<()> {
    todo!();
}

async fn show_contracts_as_json() -> anyhow::Result<()> {
    todo!();
}

async fn show_agents_as_json() -> anyhow::Result<()> {
    todo!();
}

async fn show_current_agent_as_json() -> anyhow::Result<()> {
    todo!();
}

async fn show_ships_as_json() -> anyhow::Result<()> {
    todo!();
}

async fn show_items_as_json() -> anyhow::Result<()> {
    todo!();
}

async fn show_traits_as_json() -> anyhow::Result<()> {
    todo!();
}

async fn show_engines_as_json() -> anyhow::Result<()> {
    todo!();
}

async fn show_frames_as_json() -> anyhow::Result<()> {
    todo!();
}

