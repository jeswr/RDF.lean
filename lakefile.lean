import Lake
open System Lake DSL

package RDF

@[default_target]
lean_lib RDF

extern_lib some_rust_lib (pkg : NPackage _package.name) := do
  proc { cmd := "cargo", args := #["build", "--release"], cwd := pkg.dir }
  let name := nameToSharedLib "some_rust_lib"
  let srcPath := pkg.dir / "target" / "release" / name
  IO.FS.createDirAll pkg.buildDir
  let tgtPath := pkg.buildDir / name
  IO.FS.writeBinFile tgtPath (← IO.FS.readBinFile srcPath)
  return (pure tgtPath)
