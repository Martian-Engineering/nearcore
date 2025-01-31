#!/usr/bin/env python3
"""Test echo functionality of the proof-of-response protocol between NEAR nodes."""

import sys
import pathlib
import time
import unittest

sys.path.append(str(pathlib.Path(__file__).resolve().parents[2] / 'lib'))

from configured_logger import logger
from cluster import init_cluster, start_cluster
from utils import wait_for_blocks

class ProofOfResponseTest(unittest.TestCase):
    def test_por_echo(self):
        """Test that nodes can exchange and echo messages using Proof of Response protocol."""
        logger.info("Starting proof-of-response echo test")

        nodes = start_cluster(
            num_nodes=2,
            num_observers=0,
            num_shards=1,
            config=None,
            genesis_config_changes=[],
            client_config_changes={
                0: {"network": {"por_enabled": True}},
                1: {"network": {"por_enabled": True}},
            },
        )

        # Wait for nodes to start and produce some blocks to ensure they're connected
        wait_for_blocks(nodes[0], count=5, timeout=60)
        logger.info("Nodes started and producing blocks")

        # The nodes should automatically send PoR messages to each other on startup
        # We just need to wait a bit and check their logs
        time.sleep(5)  

        # Search logs for evidence of PoR messages being sent and received
        node0_logs = nodes[0].get_logs()
        node1_logs = nodes[1].get_logs()

        # Look for expected log messages
        self.assertTrue(any("Sending PoR message" in line for line in node0_logs.split('\n')),
            "Node 0 did not send PoR message")
        self.assertTrue(any("Received PoR message" in line for line in node1_logs.split('\n')),
            "Node 1 did not receive PoR message")
        self.assertTrue(any("Sending PoR message" in line for line in node1_logs.split('\n')),
            "Node 1 did not send PoR message")
        self.assertTrue(any("Received PoR message" in line for line in node0_logs.split('\n')),
            "Node 0 did not receive PoR message")

        # Clean up
        for node in nodes:
            node.kill()

        logger.info("Proof-of-response echo test completed successfully")

if __name__ == '__main__':
    unittest.main()