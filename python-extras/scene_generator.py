import json
from dataclasses import asdict
from dataclasses import dataclass
import math

@dataclass
class Vec3:
    x: float
    y: float
    z: float

@dataclass
class Quaternion:
    x: float
    y: float
    z: float
    w: float

@dataclass
class Camera:
    position: Vec3
    rotation: Quaternion
    fov: float

@dataclass
class Light:
    color: Vec3
    intensity: float
    position: Vec3
    rotation: Quaternion
    scale: Vec3

@dataclass
class SceneObject:
    mesh: str
    material: str
    position: Vec3
    rotation: Quaternion
    scale: Vec3

from typing import List

@dataclass
class Scene:
    name: str
    camera: Camera
    lights: List[Light]
    objects: List[SceneObject]

def save_scene_to_file(scene: Scene, filepath: str):
    with open(filepath, "w") as f:
        json.dump(asdict(scene), f, indent=4)

if __name__ == "__main__":
    camera = Camera(
        position=Vec3(0.0, 2.0, -12.0),
        rotation=Quaternion(0.0, 0.0, 0.0, 1.0),
        fov=60.0
    )
    light = Light(
        color=Vec3(0.0, 0.0, 0.0),
        intensity=1.0,
        position=Vec3(0.0, 0.0, 0.0),
        rotation=Quaternion(0.0, 0.0, 0.0, 0.0),
        scale=Vec3(0.0, 0.0, 0.0)
    )
    objects = []
    for i in range(100):
        obj = SceneObject(
            mesh="cube",
            material="DiffuseColorMaterial",
            position=Vec3(i % 10 * 5.0, math.floor(i / 10) * 5.0, 0.0),
            rotation=Quaternion(0.46193978, 0.1913417, 0.1913417, 0.84462326),
            scale=Vec3(5.0, 5.0, 5.0)
        )
        objects.append(obj)

    scene = Scene(
        name = "scene_08",
        camera = camera,
        lights = [light],
        objects = objects
    )

    save_scene_to_file(scene, "scene_08.json")
    print("Scene saved to scene_08.json")

