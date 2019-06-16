defmodule Mix.Tasks.Minio.SetupTest do
  use BuffServer.MixCase, async: true
  alias Mix.Tasks

  describe "run/1" do
    test "runs successfully" do
      Minio.Setup.run([])

      assert_received {:mix_shell, :info, ["Successfully created buff-packages-test!"]}
    end
  end
end
