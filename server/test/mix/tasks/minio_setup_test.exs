defmodule Mix.Tasks.Minio.SetupTest do
  use BuffServer.MixCase, async: true

  describe "run/1" do
    test "runs successfully" do
      Mix.Tasks.Minio.Setup.run([])

      assert_received {:mix_shell, :info, ["Successfully created buff-packages-test!"]}
    end
  end
end
