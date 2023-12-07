var box;
var enableRandomMovement = true;

window.addEventListener('DOMContentLoaded', function () {
    var canvas = document.getElementById('renderCanvas');
    var engine = new BABYLON.Engine(canvas, true);

    var createScene = function () {
        var scene = new BABYLON.Scene(engine);
        scene.enablePhysics(new BABYLON.Vector3(0, -9.81, 0), new BABYLON.CannonJSPlugin());

        var camera = new BABYLON.ArcRotateCamera("Camera", Math.PI / 2, Math.PI / 2, 2, new BABYLON.Vector3(0, 0, 5), scene);
        camera.attachControl(canvas, true);

        var light = new BABYLON.HemisphericLight("light1", new BABYLON.Vector3(1, 1, 0), scene);

        box = BABYLON.MeshBuilder.CreateBox("box", {size: 2}, scene);
        box.position.y = 1;
        var boxMaterial = new BABYLON.StandardMaterial("boxMat", scene);
        boxMaterial.diffuseColor = new BABYLON.Color3(0.345, 0.396, 0.949);
        box.material = boxMaterial;
        box.physicsImpostor = new BABYLON.PhysicsImpostor(box, BABYLON.PhysicsImpostor.BoxImpostor, {
            mass: 1,
            restitution: 0.9
        }, scene);

        function createWall(position, size) {
            var wall = BABYLON.MeshBuilder.CreateBox("wall", size, scene);
            wall.position = position;
            wall.physicsImpostor = new BABYLON.PhysicsImpostor(wall, BABYLON.PhysicsImpostor.BoxImpostor, {
                mass: 0,
                restitution: 0.9
            }, scene);
        }

        var wallHeight = 4, wallDepth = 1;
        createWall(new BABYLON.Vector3(0, wallHeight / 2, -10), {height: wallHeight, width: 20, depth: wallDepth});
        createWall(new BABYLON.Vector3(0, wallHeight / 2, 10), {height: wallHeight, width: 20, depth: wallDepth});
        createWall(new BABYLON.Vector3(-10, wallHeight / 2, 0), {height: wallHeight, width: wallDepth, depth: 20});
        createWall(new BABYLON.Vector3(10, wallHeight / 2, 0), {height: wallHeight, width: wallDepth, depth: 20});
        createWall(new BABYLON.Vector3(0, -wallDepth / 2, 0), {height: wallDepth, width: 20, depth: 20});

        return scene;
    };

    var scene = createScene();

    engine.runRenderLoop(function () {
        scene.render();
    });

    window.addEventListener('resize', function () {
        engine.resize();
    });
});

window.addEventListener('keydown', function (event) {
    var movementSpeed = 0.1;
    switch (event.key) {
        case 'w':
        case 'W':
            box.position.z -= movementSpeed;
            break;
        case 's':
        case 'S':
            box.position.z += movementSpeed;
            break;
        case 'a':
        case 'A':
            box.position.x += movementSpeed;
            break;
        case 'd':
        case 'D':
            box.position.x -= movementSpeed;
            break;
    }
});

function randomMovement() {
    if (!enableRandomMovement) return;

    var randomXForce = (Math.random() - 0.5) * 2;
    var randomZForce = (Math.random() - 0.5) * 2;

    box.physicsImpostor.applyImpulse(new BABYLON.Vector3(randomXForce, 0, randomZForce), box.getAbsolutePosition());

    var randomRotY = Math.random() * Math.PI / 4;
    box.rotation.y += randomRotY;
}

setInterval(randomMovement, 500);