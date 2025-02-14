import { copyFile, readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";

const SUPPORTED_VERSIONS = [
    {
        pvn: 769,
        version: "1.21.4",
    },
    {
        pvn: 768,
        version: "1.21.2",
    },
    {
        pvn: 767,
        version: "1.21",
    },
    {
        pvn: 766,
        version: "1.20.5",
    },
    {
        pvn: 765,
        version: "1.20.3",
    },
    {
        pvn: 764,
        version: "1.20.2",
    },
    {
        pvn: 763,
        version: "1.20",
    },
    {
        pvn: 762,
        version: "1.19.4",
    },
    {
        pvn: 761,
        version: "1.19.3",
    },
    {
        pvn: 760,
        version: "1.19.1",
    },
    {
        pvn: 759,
        version: "1.19",
    },
    {
        pvn: 758,
        version: "1.18.2",
    },
    {
        pvn: 757,
        version: "1.18",
    },
    {
        pvn: 756,
        version: "1.17.1",
    },
    {
        pvn: 755,
        version: "1.17",
    },
    {
        pvn: 754,
        version: "1.16.4",
    },
    {
        pvn: 753,
        version: "1.16.3",
    },
    {
        pvn: 751,
        version: "1.16.2",
    },
    {
        pvn: 736,
        version: "1.16.1",
    },
    {
        pvn: 735,
        version: "1.16",
    },
    {
        pvn: 578,
        version: "1.15.2",
    },
    {
        pvn: 575,
        version: "1.15.1",
    },
    {
        pvn: 573,
        version: "1.15",
    },
    {
        pvn: 498,
        version: "1.14.4",
    },
    {
        pvn: 490,
        version: "1.14.3",
    },
    {
        pvn: 485,
        version: "1.14.2",
    },
    {
        pvn: 480,
        version: "1.14.1",
    },
    {
        pvn: 477,
        version: "1.14",
    },
    {
        pvn: 404,
        version: "1.13.2",
    },
    {
        pvn: 401,
        version: "1.13.1",
    },
    {
        pvn: 393,
        version: "1.13",
    },
    {
        pvn: 340,
        version: "1.12.2",
    },
    {
        pvn: 338,
        version: "1.12.1",
    },
    {
        pvn: 335,
        version: "1.12",
    },
    {
        pvn: 316,
        version: "1.11.1",
    },
    {
        pvn: 315,
        version: "1.11",
    },
    {
        pvn: 210,
        version: "1.10",
    },
    {
        pvn: 110,
        version: "1.9.3",
    },
    {
        pvn: 109,
        version: "1.9.2",
    },
    {
        pvn: 108,
        version: "1.9.1",
    },
    {
        pvn: 107,
        version: "1.9",
    },
    {
        pvn: 47,
        version: "1.8.0",
    },
    {
        pvn: 5,
        version: "1.7.6",
    },
    {
        pvn: 4,
        version: "1.7.2",
    },
];

function generateLauncherProfiles() {
    const minecraftHome = join(process.env.HOME, ".other_minecraft");
    return SUPPORTED_VERSIONS.map(({ pvn, version }) => {
        const created = new Date().toISOString();
        return [
            version.replaceAll(".", "_"),
            {
                created,
                icon: "Hardened_Clay",
                javaArgs:
                    "-Xmx512M -XX:+UnlockExperimentalVMOptions -XX:+UseG1GC -XX:G1NewSizePercent=20 -XX:G1ReservePercent=20 -XX:MaxGCPauseMillis=50 -XX:G1HeapRegionSize=32M",
                lastUsed: created,
                lastVersionId: version,
                name: `${version} (${pvn})`,
                gameDir: minecraftHome,
                resolution: {
                    height: 480,
                    width: 854,
                },
                type: "custom",
            },
        ];
    });
}

(async () => {
    const minecraftHome = join(process.env.HOME, ".minecraft");
    const launcherProfiles = join(minecraftHome, "launcher_profiles.json");
    await copyFile(launcherProfiles, `${launcherProfiles}.backup`);
    const currentProfiles = JSON.parse(
        await readFile(launcherProfiles, "utf8"),
    );
    const profilesToAdd = Object.fromEntries(generateLauncherProfiles());
    currentProfiles["profiles"] = {
        ...currentProfiles["profiles"],
        ...profilesToAdd,
    };
    await writeFile(launcherProfiles, JSON.stringify(currentProfiles, null, 2));
})();
