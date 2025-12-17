// Test SysML v2 file for extension testing
package TestPackage {
    // This is a part definition
    part def Vehicle {
        attribute mass: Real;
        attribute speed: Real;
    }

    // This is a part usage
    part myVehicle : Vehicle {
        /* Block comment test
           Multiple lines
        */
        attribute color: String = "blue";
    }
}
