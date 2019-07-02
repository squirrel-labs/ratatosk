using System;
using DSALib.Auxiliary.Calculator;
using Moq;
using NUnit.Framework;

namespace NUnitTest.Auxiliary.Calculator {
    [TestFixture]
    public class StringSolverTests {
        [SetUp]
        public void SetUp() {
            mockRepository = new MockRepository(MockBehavior.Strict);
        }

        [TearDown]
        public void TearDown() {
            mockRepository.VerifyAll();
        }

        private MockRepository mockRepository;

        private StringSolver CreateStringSolver(string input) {
            return new StringSolver(input);
        }

        [Test]
        public void Solve_braces() {
            // Arrange
            var unitUnderTest = CreateStringSolver("1+(1-4)*6+2");

            // Act
            var result = unitUnderTest.Solve();

            // Assert
            Assert.AreEqual(-15, result);
        }

        [Test]
        [MaxTime(200)]
        public void Solve_braces_timeout() {
            // Arrange
            var unitUnderTest = CreateStringSolver("1+(1-(4)*6+2");

            // Act
            Assert.Throws<ArgumentException>(() => unitUnderTest.Solve(), "Invalid brace sequence");
        }

        [Test]
        public void Solve_mult() {
            // Arrange
            var unitUnderTest = CreateStringSolver("1+1-4*6+2");

            // Act
            var result = unitUnderTest.Solve();

            // Assert
            Assert.AreEqual(-20, result);
        }

        [Test]
        public void Solve_StateUnderTest_ExpectedBehavior() {
            // Arrange
            var unitUnderTest = CreateStringSolver("1+1");

            // Act
            var result = unitUnderTest.Solve();

            // Assert
            Assert.AreEqual(2, result);
        }

        [Test]
        public void Solve_wrong_braces() {
            // Arrange
            var unitUnderTest = CreateStringSolver("1+)(1-4)*6+2");

            // Act
            Assert.Throws<ArgumentException>(() => unitUnderTest.Solve(), "Invalid brace sequence");
        }

        [Test]
        public void ToString_StateUnderTest_ExpectedBehavior() {
            // Arrange
            var unitUnderTest = CreateStringSolver("3+-4");

            // Act
            var result = unitUnderTest.ToString();

            // Assert
            Assert.AreEqual("(0+3+-4)", result);
        }
    }
}