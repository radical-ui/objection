//
//  ObjectionAppTests.swift
//  ObjectionAppTests
//
//  Created by Elijah Mooring on 8/26/24.
//

import XCTest
@testable import ObjectionApp

final class ObjectionAppTests: XCTestCase {
    func objectPathDoesMatch() throws {
        XCTAssertTrue(ObjectPath(path: "*").match(id: "foo"))
        XCTAssertTrue(ObjectPath(path: "bar/foo").match(id: "bar/*"))
        XCTAssertTrue(ObjectPath(path: "bar/foo/baz").match(id: "bar/*/baz"))
        XCTAssertTrue(ObjectPath(path: "*/foo").match(id: "bar/foo"))
        XCTAssertTrue(ObjectPath(path: "foo/bar/baz").match(id: "*/*/*"))
        
        XCTAssertFalse(ObjectPath(path: "*").match(id: "bar/baz"))
        XCTAssertFalse(ObjectPath(path: "foo").match(id: "bar"))
        XCTAssertFalse(ObjectPath(path: "foo").match(id: "foo/bar"))
        XCTAssertFalse(ObjectPath(path: "bar/foo").match(id: "foo/*"))
    }

    func testPerformanceExample() throws {
        // This is an example of a performance test case.
        measure {
            // Put the code you want to measure the time of here.
        }
    }

}
