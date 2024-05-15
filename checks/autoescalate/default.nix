{ nixosTest, escalationTool }:
nixosTest {
  name = "autoescalate-${escalationTool}";

  nodes.machine = { pkgs, lib, ... }:
    with lib; {
      imports = [
        (if escalationTool == "sudo" then {
          security.sudo = {
            enable = true;
            wheelNeedsPassword = false;
          };
        }

        else if escalationTool == "doas" then {
          security.sudo.enable = mkForce false;
          security.doas = {
            enable = true;
            wheelNeedsPassword = false;
          };
        }

        else
          builtins.throw ("Unrecognized escalation tool" ++ escalationTool))
      ];

      users.users = {
        admin = {
          isNormalUser = true;
          extraGroups = [ "wheel" ];
        };
      };

      environment.systemPackages = with pkgs; [ caligula ];
    };

  testScript = ''
    try:
        # Set up loop devices
        machine.succeed('dd if=/dev/zero of=/tmp/blockfile bs=1M count=1')
        machine.succeed('dd if=/dev/urandom of=/tmp/input.iso bs=100K count=1')
        machine.succeed('losetup /dev/loop0 /tmp/blockfile')

        # Sanity check: can we run something without asking for a password?
        machine.succeed('timeout 10 su admin -c "${escalationTool} -- echo We are able to escalate without asking for a password"')

        with subtest("should succeed when run as non-root wheel user"):
            machine.succeed('timeout 10 su admin -c "caligula burn /tmp/input.iso --force -o /dev/loop0 --hash skip --compression auto --root always --interactive never"')
    finally: 
        print(machine.execute('for x in $(find /tmp/caligula-* -type f); do echo "$x"; cat "$x"; echo; done', check_output=True)[1])
  '';
}
